module.exports = {
	entry: {
        server: './src/server.tsx',
        client: './src/client.tsx'
    },
	module: {
		rules: [
			{
				test: /\.(ts|tsx)$/, // Matches both .js, .ts and .tsx files
				exclude: [/node_modules/],
				loader: "builtin:swc-loader",
				options: {
					jsc: {
						parser: {
							syntax: "typescript", // Enables TypeScript parsing
							tsx: true, // Enables parsing of TSX syntax
						},
						target: "es2015",
					},
				},
			},
			{
				test: /\.(png|svg|jpg|jpeg|gif)$/i,
				type: "asset/resource",
			},
		],
	},
};