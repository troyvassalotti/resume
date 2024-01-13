module.exports = function (config) {
  config.addPassthroughCopy({ "./public": "/" });

  config.addFilter("mmyyyy", function (date) {
    let d = new Date(date);
    return `${d.getUTCMonth() + 1}/${d.getUTCFullYear()}`;
  });

  return {
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    dir: {
      input: "src",
      output: "dist",
    },
  };
};
