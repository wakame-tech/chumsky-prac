import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context) {
  vscode.window.showInformationMessage(
    `Extension 'vscode-language-server' is now active.`
  );
  let serverOptions: ServerOptions = {
    command: path.join("..", "lsp-server", "target", "debug", "lsp-server.exe"),
    args: [],
  };

  let clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "ipulang" }],
  };
  client = new LanguageClient("ipulang-lsp", serverOptions, clientOptions);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
