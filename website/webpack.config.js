const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const path = require('path');

module.exports = env => {
  /** @type {boolean} */
  const prod = env?.production ?? false

  return {
    entry: "./src/bootstrap.js",
    devtool: 'source-map',
    output: {
      path: path.resolve(__dirname, "dist"),
      filename: "bootstrap.js",
    },
    mode: prod ? "production" : "development",
    plugins: [
      new CleanWebpackPlugin(),
      new CopyWebpackPlugin(['index.html', 'style.css', 'ud.svg', 'sections/*']),
    ],
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
    },
  }
}
