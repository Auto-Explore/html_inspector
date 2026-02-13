# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLTrackElement.src</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function(){
  var track = document.createElement('track');
  assert_equals(track.src, '');
  assert_equals(track.getAttribute('src'), null);
}, document.title + ' missing value');

function resolve(url) {
  var link = document.createElement('a');
  link.setAttribute('href', url);
  return link.href;
}

var tests = [
  {input:'', expectedIDL:resolve(''), desc:'empty string'},
  {input:'http://foo bar', expectedIDL:'http://foo bar', desc:'unresolvable value'},
  {input:'test', expectedIDL:resolve('test'), desc:'resolvable value'},
  // Leading and trailing C0 controls and space is stripped per url spec.
  {input:'\u0000', expectedIDL:resolve(''), desc:'\\u0000'},
  {input:'foo\u0000bar', expectedIDL:resolve('foo%00bar'), desc:'foo\\u0000bar'},
];

tests.forEach(function(t) {
  test(function(){
      var track = document.createElement('track');
      track.setAttribute('src', t.input);
      assert_equals(track.src, t.expectedIDL);
      assert_equals(track.getAttribute('src'), t.input);
  }, [document.title, t.desc, 'in content attribute'].join(' '));

  test(function(){
      var track = document.createElement('track');
      track.src = t.input;
      assert_equals(track.src, t.expectedIDL);
      assert_equals(track.getAttribute('src'), t.input);
  }, [document.title, 'assigning', t.desc, 'to IDL attribute'].join(' '));
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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLTrackElement/src.html"
}
```
