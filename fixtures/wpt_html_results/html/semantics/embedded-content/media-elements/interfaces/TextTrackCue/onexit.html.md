# html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/onexit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/onexit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TextTrackCue.onexit</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
setup(function(){
    window.c1 = new VTTCue(0, 1, 'text1');
    window.ev = new Event('exit');
    window.ran = false;
    window.cb = function() { ran = true; };
});
test(function(){
    assert_equals(c1.onexit, null, 'initial value');
    c1.onexit = undefined;
    assert_equals(c1.onexit, null, 'assigning undefined');
    c1.onexit = cb;
    assert_equals(c1.onexit, cb, 'assigning onexit');
    c1.dispatchEvent(ev);
    assert_true(ran, 'dispatching event');
    c1.onexit = null;
    assert_equals(c1.onexit, null, 'assigning null');
    ran = false;
    c1.dispatchEvent(ev);
    assert_false(ran, 'dispatching event after nulling onexit');
});
test(function(){
    c1.addEventListener('exit', cb, false);
    c1.dispatchEvent(ev);
    assert_true(ran);
    c1.removeEventListener('exit', cb, false);
    ran = false;
    c1.dispatchEvent(ev);
    assert_false(ran);
}, 'TextTrackCue.addEventListener/removeEventListener');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TextTrackCue/onexit.html"
}
```
