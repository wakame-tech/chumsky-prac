import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

const selector = { language: "ipulang", scheme: "file" };

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  let serverOptions: ServerOptions = {
    command: path.join(context.extensionPath, "../target/debug/lsp-server.exe"),
    args: [],
  };

  let clientOptions: LanguageClientOptions = {
    documentSelector: [selector],
  };
  client = new LanguageClient("ipulang-lsp", serverOptions, clientOptions);

  context.subscriptions.push(client.start());
  vscode.window.showInformationMessage(
    `Extension 'vscode-language-server' is now active. ${new Date().toLocaleTimeString()}`
  );
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
