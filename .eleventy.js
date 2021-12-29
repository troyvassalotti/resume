const htmlmin = require("html-minifier-terser")

module.exports = function(config) {
    config.addPassthroughCopy({"./public": "/"})

    config.addFilter("mmyyyy", function (date) {
        let d = new Date(date)
        return `${d.getUTCMonth() + 1}/${d.getUTCFullYear()}`
    })

    if (process.env.ELEVENTY_ENV === "production") {
        config.addTransform("htmlmin", function (content, outputPath) {
            if (this.outputPath && this.outputPath.endsWith(".html")) {
                let minified = htmlmin.minify(content, {
                    useShortDoctype: true,
                    removeComments: true,
                    collapseWhitespace: true,
                    minifyCSS: true,
                    minifyJS: true,
                });
                return minified
            }
            return content
        })
    }

    return {
        markdownTemplateEngine: "njk",
        htmlTemplateEngine: "njk",
        dir: {
            input: "src",
            output: "dist"
        }
    }
}