# html/browsers/windows/browsing-context-names/choose-_blank-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/choose-_blank-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: Browsing context - `_blank` name keyword</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(t => {
  var window1 = window.open('about:blank', '_blank');
  var window2 = window.open('about:blank', '_blank');
  var window3 = window.open('about:blank', '_blank');
  t.add_cleanup(() => {
    window1.close();
    window2.close();
    window3.close();
  });
  assert_not_equals(window1, window2);
  assert_not_equals(window2, window3);
  assert_not_equals(window1, window3);
}, 'window.open into `_blank` should create a new browsing context each time');

test(t => {
  var window1 = window.open('about:blank', '_bLAnk');
  var window2 = window.open('about:blank', '_bLAnk');
  var window3 = window.open('about:blank', '_bLAnk');
  t.add_cleanup(() => {
    window1.close();
    window2.close();
    window3.close();
  });
  assert_not_equals(window1, window2);
  assert_not_equals(window2, window3);
  assert_not_equals(window1, window3);
}, '`_blank` should be ASCII case-insensitive');
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
  "source_name": "html/browsers/windows/browsing-context-names/choose-_blank-001.html"
}
```
