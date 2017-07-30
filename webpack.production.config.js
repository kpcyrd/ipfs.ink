var Webpack = require('webpack');
var ExtractTextPlugin = require('extract-text-webpack-plugin');
var AssetsPlugin = require('assets-webpack-plugin');
var path = require('path');
var assetsPath = path.resolve(__dirname, 'public', 'assets');
var entryPath = path.resolve(__dirname, 'frontend', 'app.es6.js');
var stylePath = path.resolve(__dirname, 'frontend', 'style.css');
var host = process.env.APP_HOST || 'localhost';

var config = {

    devtool: 'source-map',
    entry: {
        'bundle.js': entryPath,
        'style.css': stylePath,
    },
    output: {
        path: assetsPath,
        filename: '[chunkhash].[name]',
        publicPath: '/assets/'
    },
    module: {
        loaders: [{
            test: /\.es6\.js$/,
            loader: 'babel-loader',
            query: {
                presets: ['es2015']
            }
        }, {
            test: /\.css$/,
            loader: ExtractTextPlugin.extract('css-loader?minimize')
        }]
    },
    plugins: [
        new ExtractTextPlugin({ filename: '[chunkhash].[name]', allChunks: true }),
        new AssetsPlugin(),
    ]
};

module.exports = config;
