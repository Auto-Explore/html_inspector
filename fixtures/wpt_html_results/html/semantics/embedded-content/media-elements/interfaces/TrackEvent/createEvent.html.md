# html/semantics/embedded-content/media-elements/interfaces/TrackEvent/createEvent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/TrackEvent/createEvent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>TrackEvent created with createEvent</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    // https://www.w3.org/Bugs/Public/show_bug.cgi?id=17268
    assert_throws_dom('NOT_SUPPORTED_ERR', function() {
        var ev = document.createEvent('TrackEvent');
    });
    var ev = new TrackEvent('foo');
    assert_false('initTrackEvent' in ev, 'initTrackEvent');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/TrackEvent/createEvent.html"
}
```
