# html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute-redirect-on-load.https.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute-redirect-on-load.https.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
  <script src=/resources/testharness.js></script>
  <script src=/resources/testharnessreport.js></script>
  <script src=/resources/testdriver.js></script>
  <script src=/resources/testdriver-vendor.js></script>
  <script src=/common/media.js></script>
  <script src=/feature-policy/resources/featurepolicy.js></script>
  <script src=/feature-policy/resources/autoplay.js></script>
  <script>
  'use strict';
  const relative_path = '/feature-policy/resources/feature-policy-autoplay.html';
  const base_src = '/feature-policy/resources/redirect-on-load.html#';
  const same_origin_src = base_src + relative_path;
  const cross_origin_src = base_src + 'https://{{domains[www]}}:{{ports[https][0]}}' +
    relative_path;
  const header = 'Feature-Policy allow="autoplay"';

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability(
          'autoplay', t, same_origin_src,
          expect_feature_available_default, 'autoplay');
    });
  }, header + ' allows same-origin navigation in an iframe.');

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability(
          'autoplay', t, cross_origin_src,
          expect_feature_unavailable_default, 'autoplay');
    });
  }, header + ' disallows cross-origin navigation in an iframe.');
  </script>
</body>
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
  "source_name": "html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute-redirect-on-load.https.sub.html"
}
```
