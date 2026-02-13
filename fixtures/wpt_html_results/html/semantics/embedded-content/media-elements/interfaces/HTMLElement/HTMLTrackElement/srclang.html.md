# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/srclang.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/srclang.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLTrackElement.srclang</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var track = document.createElement('track');
    assert_equals(track.srclang, '');
    assert_equals(track.getAttribute('srclang'), null);
}, document.title + ' missing value');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('srclang', '');
    assert_equals(track.srclang, '');
    assert_equals(track.getAttribute('srclang'), '');
}, document.title + ' empty string content attribute');

test(function(){
    var track = document.createElement('track');
    track.srclang = '';
    assert_equals(track.srclang, '');
    assert_equals(track.getAttribute('srclang'), '');
}, document.title + ' empty string IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('srclang', 'foo');
    assert_equals(track.srclang, 'foo');
    assert_equals(track.getAttribute('srclang'), 'foo');
}, document.title + ' lowercase content attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('srclang', 'FOO');
    assert_equals(track.srclang, 'FOO');
    assert_equals(track.getAttribute('srclang'), 'FOO');
}, document.title + ' uppercase content attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('srclang', '\u0000');
    assert_equals(track.srclang, '\u0000');
    assert_equals(track.getAttribute('srclang'), '\u0000');
}, document.title + ' \\u0000 content attribute');

test(function(){
    var track = document.createElement('track');
    track.srclang = 'foo';
    assert_equals(track.srclang, 'foo');
    assert_equals(track.getAttribute('srclang'), 'foo');
}, document.title + ' lowercase IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.srclang = 'FOO';
    assert_equals(track.srclang, 'FOO');
    assert_equals(track.getAttribute('srclang'), 'FOO');
}, document.title + ' uppercase IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.setAttribute('srclang', ' foo \n');
    assert_equals(track.srclang, ' foo \n');
    assert_equals(track.getAttribute('srclang'), ' foo \n');
}, document.title + ' whitespace in content attribute');

test(function(){
    var track = document.createElement('track');
    track.srclang = ' foo \n';
    assert_equals(track.srclang, ' foo \n');
    assert_equals(track.getAttribute('srclang'), ' foo \n');
}, document.title + ' whitespace in IDL attribute');

test(function(){
    var track = document.createElement('track');
    track.srclang = '\u0000';
    assert_equals(track.srclang, '\u0000');
    assert_equals(track.getAttribute('srclang'), '\u0000');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/srclang.html"
}
```
