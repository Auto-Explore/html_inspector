# html/semantics/embedded-content/media-elements/seeking/seek-to-max-value.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/seeking/seek-to-max-value.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>seek to Number.MAX_VALUE</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id=log></div>
<script>
async_test(function(t) {
  var v = document.createElement('video');
  v.src = getVideoURI('/media/movie_5');
  v.onloadedmetadata = t.step_func(function() {
    assert_equals(v.seekable.length, 1);
    v.currentTime = Number.MAX_VALUE;
    assert_true(v.seeking, 'seeking after setting');
    assert_equals(v.currentTime, v.seekable.end(0), 'currentTime after setting');
    v.onseeked = t.step_func(function(e) {
      assert_false(v.seeking, 'seeking in seeked event');
      assert_equals(v.currentTime, v.seekable.end(0), 'currentTime in seeked event');
      t.done();
    });
  });
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
  "source_name": "html/semantics/embedded-content/media-elements/seeking/seek-to-max-value.htm"
}
```
