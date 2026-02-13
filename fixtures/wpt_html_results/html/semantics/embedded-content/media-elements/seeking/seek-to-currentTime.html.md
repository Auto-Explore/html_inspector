# html/semantics/embedded-content/media-elements/seeking/seek-to-currentTime.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/seeking/seek-to-currentTime.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>seek to currentTime</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id=log></div>
<script>
async_test(function(t) {
  var v = document.createElement('video');
  v.src = getVideoURI('/media/movie_5');
  v.onloadedmetadata = t.step_func(function() {
    assert_greater_than(v.readyState, v.HAVE_NOTHING, 'readyState');
    assert_greater_than(v.seekable.length, 0, 'seekable ranges');
    assert_false(v.seeking, 'seeking before setting currentTime');
    v.currentTime = v.currentTime;
    assert_true(v.seeking, 'seeking after setting currentTime');
    var events = [];
    v.onseeking = v.ontimeupdate = v.onseeked = t.step_func(function(e) {
      events.push(e.type);
      // v.seeking can be true or false in the seeking event, see
      // https://www.w3.org/Bugs/Public/show_bug.cgi?id=24774
      if (e.type != 'seeking') {
        assert_equals(v.seeking, false, 'seeking in ' + e.type + ' event');
      }
      if (e.type == 'seeked') {
        assert_array_equals(events, ['seeking', 'timeupdate', 'seeked'],
                            'fired events');
        t.done();
      }
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
  "source_name": "html/semantics/embedded-content/media-elements/seeking/seek-to-currentTime.html"
}
```
