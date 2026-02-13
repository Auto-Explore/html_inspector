# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/readyState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/readyState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLTrackElement.readyState</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var track = document.createElement('track');
    assert_equals(track.readyState, 0);
}, document.title + ' default value');

test(function(){
    assert_equals(HTMLTrackElement.NONE, 0);
    assert_equals(HTMLTrackElement.LOADING, 1);
    assert_equals(HTMLTrackElement.LOADED, 2);
    assert_equals(HTMLTrackElement.ERROR, 3);
}, document.title + ' values');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/readyState.html"
}
```
