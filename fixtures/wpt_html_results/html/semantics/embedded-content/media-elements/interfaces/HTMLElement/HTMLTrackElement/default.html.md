# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/default.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/default.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLTrackElement.default</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var track = document.createElement('track');
    assert_equals(track['default'], false);
    assert_equals(track.getAttribute('default'), null);
}, document.title + ' missing value');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('default', '');
    assert_equals(track['default'], true);
    assert_equals(track.getAttribute('default'), '');
}, document.title + ' empty string content attribute');

test(function(){
    var track = document.createElement('track');
    track['default'] = '';
    assert_equals(track['default'], false);
    assert_equals(track.getAttribute('default'), null);
}, document.title + ' empty string IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('default', 'foo');
    assert_equals(track['default'], true);
    assert_equals(track.getAttribute('default'), 'foo');
}, document.title + ' foo in content attribute');

test(function(){
    var track = document.createElement('track');
    track['default'] = 'foo';
    assert_equals(track['default'], true);
    assert_equals(track.getAttribute('default'), '');
}, document.title + ' foo in IDL attribute');

test(function(){
    var track = document.createElement('track');
    track['default'] = true;
    assert_equals(track['default'], true);
    assert_equals(track.getAttribute('default'), '');
}, document.title + ' true in IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('default', '');
    track['default'] = false;
    assert_equals(track['default'], false);
    assert_equals(track.getAttribute('default'), null);
}, document.title + ' false in IDL attribute');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/default.html"
}
```
