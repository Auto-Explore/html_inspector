# html/syntax/parsing/html_content_in_foreign_context.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/html_content_in_foreign_context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Foreign contexts with HTML tag children</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody">
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inforeign">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
test(function() {
  const contexts = ["svg", "math"];
  const elements = ["/p", "/br", "b", "big", "blockquote", "br", "center", "code", "dd", "div", "dl", "dt", "em", "embed", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "i", "img", "li", "listing", "menu", "meta", "nobr", "ol", "p", "pre", "ruby", "s", "small", "span", "strong", "strike", "sub", "sup", "table", "tt", "u", "ul", "var"];
  contexts.forEach(c => {
  	elements.forEach(e => {
  	  const wrapper = document.createElement('div');
  	  const html = `<${c}><${e}></${c}`
  	  wrapper.innerHTML = html;
  	  assert_not_equals(wrapper.innerHTML, html, "The inner HTML should get mutated");

  	  const tagname = e[0]=='/' ? e.substr(1) : e;
  	  const element = wrapper.getElementsByTagName(tagname)[0];
  	  assert_not_equals(element, undefined,`Unable to locate the ${e} node in ${c}`)
  	  const parent = element.parentNode
  	  assert_equals(element.parentNode, wrapper,`The ${e} tag did not exit the ${c}`)
  	});
  });
}, "HTML namespace nodes should exit foreign contexts");

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/parsing/html_content_in_foreign_context.html"
}
```
