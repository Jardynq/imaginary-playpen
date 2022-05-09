const path = require("path");
const dist = path.resolve(__dirname, "dist");

const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const WebpackBar = require("webpackbar");


module.exports = {
  mode: "production",
  entry: {
    index: path.resolve(__dirname, "index.js")
  },
  output: {
    path: dist,
    publicPath: '/',
    filename: "[name].js"
  },
  devServer: {
    port: 8000,
    open: false,
    host: '0.0.0.0',
  },
  performance: {
      hints: false
  },
  
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../"),
      extraArgs: "--no-typescript",
      forceWatch: true,
      forceMode: "development",

      // This does not allow for panic! console output.
      //forceMode: "production",
    }),
    new CopyPlugin([
      path.resolve(__dirname, "./index.html"),
      //path.resolve(__dirname, "./pkg"),
      //path.resolve(__dirname, "./"),
    ]),
    new WebpackBar(),
  ]
};
