# html/syntax/serializing-html-fragments/initial-linefeed-pre.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-html-fragments/initial-linefeed-pre.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>innerHTML getter for pre/textarea/listing with initial LF</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id="outer">
<div id="inner">
<pre id="pre1">
x</pre>
<pre id="pre2">

x</pre>
<textarea id="textarea1">
x</textarea>
<textarea id="textarea2">

x</textarea>
<listing id="listing1">
x</listing>
<listing id="listing2">

x</listing>
</div>
</div>

<script>
var expected_outer = '\n<div id="inner">\n<pre id="pre1">x</pre>\n<pre id="pre2">\nx</pre>\n<textarea id="textarea1">x</textarea>\n<textarea id="textarea2">\nx</textarea>\n<listing id="listing1">x</listing>\n<listing id="listing2">\nx</listing>\n</div>\n';
var expected_inner = expected_outer.replace('\n<div id="inner">', '').replace('</div>\n', '');
var expected_1 = 'x';
var expected_2 = '\nx';

test(function() {
  assert_equals(outer.innerHTML, expected_outer);
}, 'outer div');

test(function() {
  assert_equals(inner.innerHTML, expected_inner);
}, 'inner div');

['pre', 'textarea', 'listing'].forEach(function(tag) {
  test(function() {
    assert_equals(document.getElementById(tag + '1').innerHTML, expected_1);
  }, tag + '1');

  test(function() {
    assert_equals(document.getElementById(tag + '2').innerHTML, expected_2);
  }, tag + '2');
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
  "source_name": "html/syntax/serializing-html-fragments/initial-linefeed-pre.html"
}
```
