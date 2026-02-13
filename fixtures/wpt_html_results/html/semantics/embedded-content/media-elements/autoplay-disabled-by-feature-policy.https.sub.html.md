# html/semantics/embedded-content/media-elements/autoplay-disabled-by-feature-policy.https.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/autoplay-disabled-by-feature-policy.https.sub.html",
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
  const header = 'Feature-Policy header: autoplay "none"';

  async_test(t => {
    simulateGesture(t, () => {
      isAutoplayAllowed().then(t.step_func_done((result) => {
        assert_true(result);
      }));
    });
  }, header + ' has no effect on the top level document.');

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability('autoplay', t, same_origin_src,
          expect_feature_unavailable_default);
    });
  }, header + ' disallows same-origin iframes.');

  async_test(t => {
    simulateGesture(t, () => {
      test_feature_availability('autoplay', t, cross_origin_src,
          expect_feature_unavailable_default,);
    });
  }, header + ' disallows cross-origin iframes.');
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
  "source_name": "html/semantics/embedded-content/media-elements/autoplay-disabled-by-feature-policy.https.sub.html"
}
```
