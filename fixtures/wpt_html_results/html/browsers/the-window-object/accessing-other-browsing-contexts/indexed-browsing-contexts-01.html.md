# html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: the browsing contexts must be sorted in the order that their containers were inserted into the Document</title>
<link rel="author" title="Intel" href="http://www.intel.com/" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/browsers.html#accessing-other-browsing-contexts" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

var t1 = async_test("The window's length must return the number of child browsing contexts(in iframe)");
function on_load1(fr) {
  t1.step(function () {
    var doc = fr.contentDocument;
    var fr3 = doc.createElement("iframe");
    fr3.setAttribute("id", "fr3");
    doc.body.insertBefore(fr3, doc.getElementById("tbl"));

    assert_equals(fr.contentWindow.length, 3, "The window.length should be 3.");
    assert_array_equals([fr.contentWindow[0].frameElement, fr.contentWindow[1].frameElement, fr.contentWindow[2].frameElement],
                        [fr.contentDocument.getElementById("fr4"), fr.contentDocument.getElementById("fr5"), fr.contentDocument.getElementById("fr3")],
                        "The child browsing contexts must be sorted in the order that their containers were inserted into the Document.");
  });
  t1.done();
}

var t2 = async_test("The window's length must return zero if it has no child browsing context");
function on_load2(fr) {
  t2.step(function () {
    assert_equals(fr.contentWindow.length, 0, "The window.length should be 0.");
  });
  t2.done();
}

</script>
<iframe id="fr1" src="test1.html" style="display:none" onload="on_load1(this)"></iframe>
<iframe id="fr2" src="test2.html" style="display:none" onload="on_load2(this)"></iframe>
<script>

test(function () {
  assert_equals(window.length, 2, "The window.length should be 2.");
  assert_array_equals([window[0].frameElement, window[1].frameElement],
                      [document.getElementById("fr1"), document.getElementById("fr2")],
                     "The child browsing contexts must be sorted in the tree order.");
}, "The window's length must return the number of child browsing contexts");

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
  "source_name": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-01.html"
}
```
