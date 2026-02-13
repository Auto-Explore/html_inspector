# html/browsers/origin/inheritance/about-srcdoc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/inheritance/about-srcdoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>about:srcdoc aliases security origin</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      test(() => {
        let iframe = document.createElement('iframe');
        iframe.srcdoc = '<body></body>';
        document.body.appendChild(iframe);
        // Should not throw: srcdoc should always be same-origin.
        iframe.contentWindow.document.body.innerHTML = '<p>Hello world!</p>';

        // Explicitly set `domain` component of origin: any other same-origin
        // browsing contexts are now cross-origin unless they also explicitly
        // set document.domain to the same value.
        document.domain = document.domain;
        // Should not throw: the origin should be aliased, so setting
        // document.domain in one Document should affect both Documents.
        assert_equals(
            iframe.contentWindow.document.body.textContent,
            'Hello world!');
      });
    </script>
  </body>
</html>
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
  "source_name": "html/browsers/origin/inheritance/about-srcdoc.html"
}
```
