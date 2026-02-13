# html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#attr-meta-http-equiv">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="meta@http-equiv values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function() {
  let loaded = 0;

  // we use a message rather than the iframe’s load event to avoid dealing with
  // spurious load events that some browsers dispatch on the initial about:blank
  addEventListener("message", this.step_func(event => {
    if (++loaded == 3) {
      const iframe = document.querySelectorAll("iframe");

      assert_equals(iframe[0].contentWindow.inline,
        undefined, "lowercase valid");
      assert_equals(iframe[1].contentWindow.inline,
        undefined, "mixed case valid");
      assert_equals(iframe[2].contentWindow.inline,
        true, "non-ASCII invalid");

      this.done();
    }
  }));
}, "keyword content-security-policy");
</script>
<iframe src="http-equiv-enumerated-ascii-case-insensitive-lower.html"></iframe>
<iframe src="http-equiv-enumerated-ascii-case-insensitive-mixed.html"></iframe>
<iframe src="http-equiv-enumerated-ascii-case-insensitive-other.html"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-meta-element/pragma-directives/http-equiv-enumerated-ascii-case-insensitive.html"
}
```
