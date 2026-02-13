# html/semantics/embedded-content/media-elements/interfaces/TextTrack/constants.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/constants.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack constants</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    window.t1 = video.addTextTrack('subtitles');
});
test(function(){
    assert_equals(t1.DISABLED, undefined, "t1.DISABLED");
    assert_equals(t1.HIDDEN, undefined, "t1.HIDDEN");
    assert_equals(t1.SHOWING, undefined, "t1.SHOWING");
    assert_equals(TextTrack.prototype.DISABLED, undefined, "TextTrack.prototype.DISABLED");
    assert_equals(TextTrack.prototype.HIDDEN, undefined, "TextTrack.prototype.HIDDEN");
    assert_equals(TextTrack.prototype.SHOWING, undefined, "TextTrack.prototype.SHOWING");
    assert_equals(TextTrack.DISABLED, undefined, "TextTrack.DISABLED");
    assert_equals(TextTrack.HIDDEN, undefined, "TextTrack.HIDDEN");
    assert_equals(TextTrack.SHOWING, undefined, "TextTrack.SHOWING");
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/constants.html"
}
```
