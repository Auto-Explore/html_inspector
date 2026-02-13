# html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-dialogs-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-dialogs-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>stable state in dialogs</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
['alert', 'confirm', 'prompt'].forEach(function(dialog) {
  test(function() {
    v = document.createElement('video');
    v.src = 'data:,';
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState before dialog');
    assert_equals(v.currentSrc, '', 'currentSrc before dialog');
    window[dialog]('dismiss this dialog');
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after dialog');
    assert_equals(v.currentSrc, '', 'currentSrc after dialog');
  }, 'stable state in ' + dialog + '()');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-dialogs-manual.html"
}
```
