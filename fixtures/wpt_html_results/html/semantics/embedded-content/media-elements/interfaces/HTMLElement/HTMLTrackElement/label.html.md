# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/label.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLTrackElement.label</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var track = document.createElement('track');
    assert_equals(track.label, '');
    assert_equals(track.getAttribute('label'), null);
}, document.title + ' missing value');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('label', '');
    assert_equals(track.label, '');
    assert_equals(track.getAttribute('label'), '');
}, document.title + ' empty string content attribute');

test(function(){
    var track = document.createElement('track');
    track.label = '';
    assert_equals(track.label, '');
    assert_equals(track.getAttribute('label'), '');
}, document.title + ' empty string IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('label', 'foo');
    assert_equals(track.label, 'foo');
    assert_equals(track.getAttribute('label'), 'foo');
}, document.title + ' lowercase content attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('label', 'FOO');
    assert_equals(track.label, 'FOO');
    assert_equals(track.getAttribute('label'), 'FOO');
}, document.title + ' uppercase content attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('label', '\u0000');
    assert_equals(track.label, '\u0000');
    assert_equals(track.getAttribute('label'), '\u0000');
}, document.title + '\\u0000 in content attribute');

test(function(){
    var track = document.createElement('track');
    track.label = 'foo';
    assert_equals(track.label, 'foo');
    assert_equals(track.getAttribute('label'), 'foo');
}, document.title + ' lowercase IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.label = 'FOO';
    assert_equals(track.label, 'FOO');
    assert_equals(track.getAttribute('label'), 'FOO');
}, document.title + ' uppercase IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('label', ' foo \n');
    assert_equals(track.label, ' foo \n');
    assert_equals(track.getAttribute('label'), ' foo \n');
}, document.title + ' whitespace in content attribute');

test(function(){
    var track = document.createElement('track');
    track.label = ' foo \n';
    assert_equals(track.label, ' foo \n');
    assert_equals(track.getAttribute('label'), ' foo \n');
}, document.title + ' whitespace in IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.label = '\u0000';
    assert_equals(track.label, '\u0000');
    assert_equals(track.getAttribute('label'), '\u0000');
}, document.title + ' \\u0000 in IDL attribute');

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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/label.html"
}
```
