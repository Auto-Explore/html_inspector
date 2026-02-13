# html/semantics/embedded-content/the-iframe-element/iframe-nosrc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-nosrc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Check processing of iframe without src and srcdoc attribute</title>
<link rel="author" title="Xidorn Quan" href="https://www.upsuper.org">
<link rel="author" title="Mozilla" href="https://www.mozilla.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>
<iframe></iframe>
<script>
  let iframe = document.querySelector("iframe");

  async_test(t => {
    let originDoc = iframe.contentDocument;
    window.addEventListener("load", t.step_func_done(() => {
      assert_equals(iframe.contentDocument, originDoc, "contentDocument shouldn't be changed");
    }));
  }, "iframe.contentDocument should not be changed");

  async_test(t => {
    iframe.addEventListener("load", t.unreached_func());
    window.addEventListener("load", () => t.done());
  }, "load event of iframe should not be fired after processing the element");
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-nosrc.html"
}
```
