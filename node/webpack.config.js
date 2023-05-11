const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = {
  entry: './nodesrc/app.jsx',
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.jsx$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
        options: { presets: ["@babel/preset-env", "@babel/preset-react"]}
      },
      {
        test: /\.css$/,
        use: [
          {loader: 'style-loader'},
          {loader: 'css-loader'}
        ]
      }
    ]
  },
  resolve: {
    extensions: [".jsx", ".js"]
  },
  plugins: [new HtmlWebpackPlugin({
    minify: false,
  })],
  devServer: {},
  output: {
    path: path.resolve(__dirname, '..', 'docs'),
    filename: "bundle.js"
  }
}