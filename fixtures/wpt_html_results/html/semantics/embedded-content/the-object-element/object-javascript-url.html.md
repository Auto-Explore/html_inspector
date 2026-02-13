# html/semantics/embedded-content/the-object-element/object-javascript-url.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-javascript-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>object - javascript: URL</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-object-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  // the steps to (re)determine what the object element represents
  // fetches url, which results in a network error
  // and thus "fallback".
  promise_test(async function (t) {
    const object = document.createElement('object');
    object.data = 'javascript:"foo"';
    object.onload = t.unreached_func('No load event expected');
    document.body.append(object);
    t.add_cleanup(() => { object.remove(); });
    await new Promise(resolve => { t.step_timeout(resolve, 100); });
  }, "javascript: in data attribute should do nothing");

  function insertObjectNavigable(t) {
    const object = document.createElement('object');
    object.data = '/resources/blank.html';
    document.body.append(object);
    t.add_cleanup(() => { object.remove(); });
    return object;
  }

  promise_test(async function (t) {
    const object = insertObjectNavigable(t);
    await new Promise(resolve => { object.onload = resolve; });
    const loaded = new Promise(resolve => { object.onload = resolve; });
    window[0].location.href = 'javascript:"test"';
    await loaded;
  }, 'location.href = \'javascript:"test"\' should fire a load event');

  promise_test(async function (t) {
    const object = insertObjectNavigable(t);
    await new Promise(resolve => { object.onload = resolve; });
    object.onload = t.unreached_func('No second load event expected');
    window[0].location.href = 'javascript:1';
    await new Promise(resolve => { t.step_timeout(resolve, 100); });
  }, 'location.href = \'javascript:1\' should not fire a load event');
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-javascript-url.html"
}
```
