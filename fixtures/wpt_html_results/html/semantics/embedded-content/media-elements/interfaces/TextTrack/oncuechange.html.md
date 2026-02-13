# html/semantics/embedded-content/media-elements/interfaces/TextTrack/oncuechange.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/oncuechange.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrack.oncuechange</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.video = document.createElement('video');
    window.t1 = video.addTextTrack('subtitles');
    window.ev = new Event('cuechange');
    window.ran = false;
    window.cb = function() { ran = true; };
});
test(function(){
    assert_equals(t1.oncuechange, null);
    t1.oncuechange = cb;
    t1.dispatchEvent(ev);
    assert_true(ran);
    t1.oncuechange = null;
    ran = false;
    t1.dispatchEvent(ev);
    assert_false(ran);
});
test(function(){
    t1.addEventListener('cuechange', cb, false);
    t1.dispatchEvent(ev);
    assert_true(ran);
    t1.removeEventListener('cuechange', cb, false);
    ran = false;
    t1.dispatchEvent(ev);
    assert_false(ran);
}, 'TextTrack.addEventListener/removeEventListener');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrack/oncuechange.html"
}
```
