# html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute.https.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute.https.sub.html",
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
  const same_origin_src = '/feature-policy/resources/feature-policy-autoplay.html';
  const cross_origin_src = 'https://{{domains[www]}}:{{ports[https][0]}}' +
    same_origin_src;
  const feature_name = 'Feature policy "autoplay"';
  const header = 'allow="autoplay" attribute';

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability(
          'autoplay', t, same_origin_src,
          expect_feature_available_default, 'autoplay');
    });
  }, feature_name + ' can be enabled in same-origin iframe using ' + header);

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability(
          'autoplay', t, cross_origin_src,
          expect_feature_available_default, 'autoplay');
    });
  }, feature_name + ' can be enabled in cross-origin iframe using ' + header);
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
  "source_name": "html/semantics/embedded-content/media-elements/autoplay-allowed-by-feature-policy-attribute.https.sub.html"
}
```
