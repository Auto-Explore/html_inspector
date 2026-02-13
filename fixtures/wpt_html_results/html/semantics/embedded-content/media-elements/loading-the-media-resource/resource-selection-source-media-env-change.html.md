# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media-env-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media-env-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>the &lt;source> media attribute: no reaction to environment change</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<iframe src="resources/media-min-width.html" width="300"></iframe>
<script>
// promises for the iframed test to resolve
let beforeEnvChange = new Promise((resolve, reject) => {
  window[0].resolveBeforeEnvChange = resolve;
});
let afterEnvChange = new Promise((resolve, reject) => {
  window[0].resolveAfterEnvChange = resolve;
});
let afterLoadCalled = new Promise((resolve, reject) => {
  window[0].resolveAfterLoadCalled = resolve;
});
const t = promise_test(async () => {
  [beforeEnvChange, afterEnvChange, afterLoadCalled] = await Promise.all([ beforeEnvChange, afterEnvChange, afterLoadCalled ]);
  assert_equals(beforeEnvChange, '#a', 'beforeEnvChange');
  assert_equals(afterEnvChange, '#a', 'afterEnvChange');
  assert_equals(afterLoadCalled, '#b', 'afterLoadCalled');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media-env-change.html"
}
```
