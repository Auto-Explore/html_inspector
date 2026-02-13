# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/crossOrigin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/crossOrigin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLMediaElement.crossOrigin</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
    var video = document.createElement('video');
    assert_true('crossOrigin' in video);
});
test(function(){
    var video = document.createElement('video');
    assert_equals(video.crossOrigin, null);
}, document.title+', content attribute missing');
test(function(){
    var video = document.createElement('video');
    video.setAttribute('crossorigin', 'foo');
    assert_equals(video.crossOrigin, 'anonymous');
}, document.title+', content attribute invalid value');
test(function(){
    var video = document.createElement('video');
    video.setAttribute('crossorigin', '');
    assert_equals(video.crossOrigin, 'anonymous');
}, document.title+', content attribute empty string');
test(function(){
    var video = document.createElement('video');
    video.setAttribute('crossorigin', 'ANONYMOUS');
    assert_equals(video.crossOrigin, 'anonymous');
}, document.title+', content attribute uppercase ANONYMOUS');
test(function(){
    var video = document.createElement('video');
    video.setAttribute('crossorigin', 'use-credentials');
    assert_equals(video.crossOrigin, 'use-credentials');
}, document.title+', content attribute use-credentials');
test(function(){
    var video = document.createElement('video');
    video.crossOrigin = '';
    assert_equals(video.getAttribute('crossorigin'), '');
}, document.title+', setting to empty string');
test(function(){
    var video = document.createElement('video');
    video.crossOrigin = null;
    assert_false(video.hasAttribute('crossorigin'));
}, document.title+', setting to null');
test(function(){
    var video = document.createElement('video');
    video.crossOrigin = 'foo';
    assert_equals(video.getAttribute('crossorigin'), 'foo');
}, document.title+', setting to invalid value');
test(function(){
    var video = document.createElement('video');
    video.crossOrigin = 'ANONYMOUS';
    assert_equals(video.getAttribute('crossorigin'), 'ANONYMOUS');
}, document.title+', setting to uppercase ANONYMOUS');
test(function(){
    var video = document.createElement('video');
    video.crossOrigin = 'use-credentials';
    assert_equals(video.getAttribute('crossorigin'), 'use-credentials');
}, document.title+', setting to use-credentials');
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/crossOrigin.html"
}
```
