# html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-beforeunload-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-beforeunload-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>stable state in beforeunload</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<button>click this button and cancel navigation</button>
<a href="data:text/plain,FAIL: did not cancel navigation"></a>
<script>
async_test(function(t) {
  window.onbeforeunload = t.step_func(function(event) {
    var message = "foo bar";
    event.returnValue = message;
    return message;
  });
  var button = document.querySelector('button');
  var link = document.querySelector('a');
  button.onclick = t.step_func(function() {
    v = document.createElement('video');
    v.src = 'data:,';
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState before dialog');
    assert_equals(v.currentSrc, '', 'currentSrc before dialog');
    link.click();
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after dialog');
    assert_equals(v.currentSrc, '', 'currentSrc after dialog');
    t.done();
    window.onbeforeonload = null;
    button.remove();
  });
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “data:text/plain,FAIL: did not cancel navigation” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 300,
        "byte_start": 242,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/stable-state-beforeunload-manual.html"
}
```
