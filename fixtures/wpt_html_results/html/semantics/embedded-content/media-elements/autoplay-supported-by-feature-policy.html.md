# html/semantics/embedded-content/media-elements/autoplay-supported-by-feature-policy.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/autoplay-supported-by-feature-policy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test that autoplay is advertised in the feature list</title>
<link rel="help" href="https://w3c.github.io/webappsec-feature-policy/#dom-featurepolicy-features">
<link rel="help" href="https://html.spec.whatwg.org/multipage/infrastructure.html#policy-controlled-features">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(() => {
    assert_in_array('autoplay', document.featurePolicy.features());
}, 'document.featurePolicy.features should advertise autoplay.');
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
  "source_name": "html/semantics/embedded-content/media-elements/autoplay-supported-by-feature-policy.html"
}
```
