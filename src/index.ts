import type { JsPlugin, PluginTransformHookParam } from '@farmfe/core';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import * as parser from "@babel/parser";
import traverse from "@babel/traverse";
import * as t from "@babel/types";
import generate from "@babel/generator";
import typescript from "@babel/preset-typescript";
interface Options {
  /* Your options here */
}

export default function farmPlugin(options?: Options): JsPlugin {
  const moduleTypes = ['ts', 'js', 'cjs', 'mjs', 'mts', 'cts'];
  const resolvedPaths: string[] = [];
  return {
    name: 'farm-js-plugin-replace-dirname',
    transform: {
      filters: {
        moduleTypes,
        resolvedPaths
      },
      async executor(param: PluginTransformHookParam) {
        const { content, resolvedPath, moduleType } = param;

        if (/\/node_modules\//.test(resolvedPath)) {
          return {
            content,
            moduleType
          }
        }

        const dirPath = path.dirname(resolvedPath);
        const ast = parser.parse(content, { sourceType: "unambiguous", plugins: ["typescript"] });

        traverse(ast, {
          Identifier(path: any) {
            if (path.node.name === "__dirname") {
              path.replaceWith(t.stringLiteral(dirPath));
            } else if (path.node.name === "__filename") {
              path.replaceWith(t.stringLiteral(resolvedPath));
            }
          },
          MetaProperty(path: any) {
            if (path.node.meta.name === 'import' && path.node.property.name === 'meta') {
              const parent = path.parentPath;
              if (parent.node.type === 'MemberExpression' && parent.node.property.name === 'url') {
                const fileUrl = pathToFileURL(resolvedPath).toString();
                parent.replaceWith(t.stringLiteral(fileUrl));
              }
            }
          },
        });
        const { code } = generate(ast);

        return {
          content: code,
          moduleType
        };
      }
    }
  };
}
