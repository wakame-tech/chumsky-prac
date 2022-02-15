import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

const tokenTypes = ["variable"];
const tokenModifiers = ["declaration"];
const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

const provider: vscode.DocumentSemanticTokensProvider = {
  provideDocumentSemanticTokens(
    document: vscode.TextDocument
  ): vscode.ProviderResult<vscode.SemanticTokens> {
    // analyze the document and return semantic tokens

    const tokensBuilder = new vscode.SemanticTokensBuilder(legend);
    // on line 1, characters 1-5 are a class declaration
    tokensBuilder.push(
      new vscode.Range(new vscode.Position(1, 1), new vscode.Position(1, 5)),
      "class",
      ["declaration"]
    );
    return tokensBuilder.build();
  },
};

const selector = { language: "ipulang", scheme: "file" };

vscode.languages.registerDocumentSemanticTokensProvider(
  selector,
  provider,
  legend
);

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
