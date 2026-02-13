# html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>parsing a hash-name reference for img and object</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
 body { margin-top: 0 }
 iframe { height: 600px; width:50px; border-top: none }
</style>

<div id="log"></div>

<iframe data-name="HTML (standards)" src="hash-name-reference-test-data.html?pipe=sub&amp;doctype=html"></iframe>
<iframe data-name="HTML (quirks)" src="hash-name-reference-test-data.html?pipe=sub&amp;doctype=quirks"></iframe>
<iframe data-name="XHTML" src="hash-name-reference-test-data.html?pipe=sub|header(Content-Type, application/xhtml%2Bxml)&amp;doctype=html"></iframe>

<script>
setup({explicit_done: true});

onload = function() {
  var iframes = document.querySelectorAll('iframe');
  iframes.forEach(function(iframe) {
    var iframeName = iframe.getAttribute('data-name');
    var doc = iframe.contentDocument;
    var divs = doc.querySelectorAll('div[data-expect]');
    var div, img, object;
    for (var i = 0; i < divs.length; ++i) {
      div = divs[i];
      img = div.querySelector('img');
      object = div.querySelector('object');
      [img, object].forEach(function(elm) {
        test(function(t) {
          var expected = div.getAttribute('data-expect');
          var expected_elm = (expected === 'no match' || elm === object) ? elm : div.querySelector('area[href="#' + expected + '"]');
          var got_elm = doc.elementFromPoint(elm.offsetLeft, elm.offsetTop);
          assert_not_equals(expected_elm, null, 'sanity check (data-expect value wrong?)');
          assert_not_equals(got_elm, null, 'sanity check (too many tests to fit in viewport?)');
          assert_equals(got_elm, expected_elm);
        }, iframeName + ' ' + elm.tagName + ' usemap=' + format_value(elm.useMap));
      });
    }
  });
  done();
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “hash-name-reference-test-data.html?pipe=sub|header(Content-Type, application/xhtml%2Bxml)&doctype=html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 695,
        "byte_start": 556,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference.html"
}
```
