import * as vscode from 'vscode';
import { exec } from 'child_process';
import * as path from 'path';

/**
 * @typedef {Object} Optimization
 * @property {string} rule_name
 * @property {string} language
 * @property {number} line_number
 * @property {string} original_code
 * @property {string} suggested_code
 * @property {string} explanation
 * @property {string} severity
 * @property {number} confidence
 */

export function activate(context) {
    console.log('🚀 Code Optimizer extension is now active!');

    // Create diagnostic collection for showing problems
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('code-optimizer');
    context.subscriptions.push(diagnosticCollection);

    // Register command: Analyze Current File
    let analyzeFileCommand = vscode.commands.registerCommand('codeOptimizer.analyzeFile', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('No active editor found!');
            return;
        }

        await analyzeDocument(editor.document, diagnosticCollection);
    });

    // Register command: Analyze Workspace
    let analyzeWorkspaceCommand = vscode.commands.registerCommand('codeOptimizer.analyzeWorkspace', async () => {
        vscode.window.showInformationMessage('🔍 Analyzing workspace...');
        
        const files = await vscode.workspace.findFiles('**/*.{js,ts,py,rs}', '**/node_modules/**');
        let totalOptimizations = 0;

        for (const file of files) {
            const document = await vscode.workspace.openTextDocument(file);
            const optimizations = await analyzeDocument(document, diagnosticCollection);
            totalOptimizations += optimizations;
        }

        vscode.window.showInformationMessage(
            `✅ Workspace analysis complete! Found ${totalOptimizations} optimization opportunities.`
        );
    });

    // Register command: Show Configuration
    let showConfigCommand = vscode.commands.registerCommand('codeOptimizer.showConfig', () => {
        const config = vscode.workspace.getConfiguration('codeOptimizer');
        const enabledRules = config.get('enabledRules');
        const minConfidence = config.get('minimumConfidence');
        
        vscode.window.showInformationMessage(
            `🎛️ Code Optimizer Config:\n` +
            `📊 Enabled Rules: ${Object.keys(enabledRules || {}).length}\n` +
            `🎯 Min Confidence: ${minConfidence}\n` +
            `⚙️ Click to open settings`,
            'Open Settings'
        ).then(selection => {
            if (selection === 'Open Settings') {
                vscode.commands.executeCommand('workbench.action.openSettings', 'codeOptimizer');
            }
        });
    });

    // Auto-analyze on file save
    let onSaveListener = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (isSupported(document.languageId)) {
            await analyzeDocument(document, diagnosticCollection);
        }
    });

    // Auto-analyze on file open
    let onOpenListener = vscode.workspace.onDidOpenTextDocument(async (document) => {
        if (isSupported(document.languageId)) {
            await analyzeDocument(document, diagnosticCollection);
        }
    });

    context.subscriptions.push(
        analyzeFileCommand,
        analyzeWorkspaceCommand,
        showConfigCommand,
        onSaveListener,
        onOpenListener
    );
}

async function analyzeDocument(document, diagnosticCollection) {
    if (!isSupported(document.languageId)) {
        return 0;
    }

    try {
        // For now, we'll simulate calling our Rust analyzer
        // In a real implementation, this would call our Rust binary
        const optimizations = await simulateRustAnalysis(document);
        
        // Convert optimizations to VS Code diagnostics
        const diagnostics = optimizations.map(opt => {
            const line = Math.max(0, opt.line_number - 1); // VS Code uses 0-based indexing
            const range = new vscode.Range(line, 0, line, document.lineAt(line).text.length);
            
            const diagnostic = new vscode.Diagnostic(
                range,
                `${opt.explanation} (${Math.round(opt.confidence * 100)}% confidence)`,
                getSeverity(opt.severity)
            );
            
            diagnostic.code = opt.rule_name;
            diagnostic.source = 'Code Optimizer';
            
            return diagnostic;
        });

        diagnosticCollection.set(document.uri, diagnostics);
        
        if (optimizations.length > 0) {
            vscode.window.showInformationMessage(
                `🔍 Found ${optimizations.length} optimization opportunities in ${path.basename(document.fileName)}`
            );
        }

        return optimizations.length;
    } catch (error) {
        vscode.window.showErrorMessage(`Code Optimizer error: ${error}`);
        return 0;
    }
}

function isSupported(languageId) {
    return ['javascript', 'javascriptreact', 'typescript', 'typescriptreact', 'python', 'rust'].includes(languageId);
}

function getSeverity(severity) {
    switch (severity) {
        case 'Error': return vscode.DiagnosticSeverity.Error;
        case 'Warning': return vscode.DiagnosticSeverity.Warning;
        case 'Info': return vscode.DiagnosticSeverity.Information;
        default: return vscode.DiagnosticSeverity.Hint;
    }
}

// Simulate calling our Rust analyzer (in real implementation, call actual binary)
async function simulateRustAnalysis(document) {
    const config = vscode.workspace.getConfiguration('codeOptimizer');
    const minConfidence = config.get<number>('minimumConfidence', 0.6);
    
    // Simulate some optimizations based on the document content
    const optimizations = [];
    const text = document.getText();
    const lines = text.split('\n');
    
    lines.forEach((line, index) => {
        // Simulate JavaScript optimizations
        if (['javascript', 'javascriptreact', 'typescript', 'typescriptreact'].includes(document.languageId)) {
            if (line.includes('let ') && line.includes('=') && !line.includes('let i')) {
                optimizations.push({
                    rule_name: 'use-const',
                    language: 'JavaScript',
                    line_number: index + 1,
                    original_code: line.trim(),
                    suggested_code: line.replace('let ', 'const '),
                    explanation: "Use 'const' for variables that never change",
                    severity: 'Info',
                    confidence: 0.8
                });
            }
            
            if (line.includes('console.log(')) {
                optimizations.push({
                    rule_name: 'no-console',
                    language: 'JavaScript',
                    line_number: index + 1,
                    original_code: line.trim(),
                    suggested_code: line.replace('console.log(', '// console.log('),
                    explanation: 'Remove console.log statements in production code',
                    severity: 'Warning',
                    confidence: 0.9
                });
            }
        }
        
        // Simulate Python optimizations
        if (document.languageId === 'python') {
            if (line.includes('.format(')) {
                optimizations.push({
                    rule_name: 'use-f-strings',
                    language: 'Python',
                    line_number: index + 1,
                    original_code: line.trim(),
                    suggested_code: line.replace('.format(', 'f"'),
                    explanation: 'Use f-strings instead of .format() for better performance',
                    severity: 'Info',
                    confidence: 0.7
                });
            }
            
            if (line.includes('print(')) {
                optimizations.push({
                    rule_name: 'no-print-debug',
                    language: 'Python',
                    line_number: index + 1,
                    original_code: line.trim(),
                    suggested_code: line.replace('print(', '# print('),
                    explanation: 'Remove print statements in production code',
                    severity: 'Warning',
                    confidence: 0.8
                });
            }
        }
    });
    
    // Filter by minimum confidence
    return optimizations.filter(opt => opt.confidence >= minConfidence);
}

export function deactivate() {
    console.log('👋 Code Optimizer extension is now deactivated');
}