# html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-script-disabled-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-script-disabled-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<title>Iframes with loading='lazy' in script disabled iframe are not handled
       as 'lazy'</title>
<link rel="help" href="https://github.com/scott-little/lazyload">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>

<div style="height:1000vh;"></div>
<iframe id="iframe" sandbox="allow-same-origin"
        src="resources/iframe-loading-lazy-in-viewport.html">
</iframe>
<script>
promise_test(async t => {
  await new Promise(resolve => iframe.addEventListener("load", resolve));

  const inner_iframe = iframe.contentDocument.querySelector("iframe");

  assert_equals(inner_iframe.contentDocument.body.textContent.trim(), 'Subframe',
              "lazy-load iframe shouldn't be honored in script disabled iframe");
});
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-loading-lazy-in-script-disabled-iframe.html"
}
```
