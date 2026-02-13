# html/dom/documents/dom-tree-accessors/nameditem-02.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: iframes</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<iframe name="test1"></iframe>

<iframe name="test2"></iframe>
<iframe name="test2"></iframe>

<iframe name="test3"></iframe>
<img name="test3">

<img name="test4">
<iframe name="test4"></iframe>

<iframe id="test5"></iframe>

<iframe name="test6" id="fail"></iframe>

<iframe name="fail" id="test7"></iframe>

<iframe name="42"></iframe>

<iframe name="test9" id="test9"></iframe>

<iframe></iframe>

<iframe name="test11a"></iframe>

<iframe name="test12"></iframe>
</div>
<script>
test(function() {
  var iframe = document.getElementsByTagName("iframe")[0];
  assert_equals(iframe.name, "test1");

  assert_true("test1" in document, '"test1" in document should be true');
  assert_equals(document.test1, iframe.contentWindow);
}, "If the only named item is an iframe, the contentWindow should be returned.");

test(function() {
  var iframe1 = document.getElementsByTagName("iframe")[1];
  assert_equals(iframe1.name, "test2");
  var iframe2 = document.getElementsByTagName("iframe")[2];
  assert_equals(iframe2.name, "test2");

  assert_true("test2" in document, '"test2" in document should be true');
  var collection = document.test2;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [iframe1, iframe2]);
}, "If there are two iframes, a collection should be returned.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[3];
  assert_equals(iframe.name, "test3");
  var img = document.getElementsByTagName("img")[0];
  assert_equals(img.name, "test3");

  assert_true("test3" in document, '"test3" in document should be true');
  var collection = document.test3;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [iframe, img]);
}, "If there are an iframe and another element (iframe first), a collection should be returned.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[4];
  assert_equals(iframe.name, "test4");
  var img = document.getElementsByTagName("img")[1];
  assert_equals(img.name, "test4");

  assert_true("test4" in document, '"test4" in document should be true');
  var collection = document.test4;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [img, iframe]);
}, "If there are an iframe and another element (iframe last), a collection should be returned.");

test(function() {
  assert_false("test5" in document, '"test5" in document should be false');
  assert_equals(document.test5, undefined);
}, "If an iframe has an id and no name, it should not be returned.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[6];
  assert_equals(iframe.name, "test6");

  assert_true("test6" in document, '"test6" in document should be true');
  assert_equals(document.test6, iframe.contentWindow);
}, "If an iframe has a name and a different id, it should be returned by its name.");

test(function() {
  assert_false("test7" in document, '"test7" in document should be false');
  assert_equals(document.test7, undefined);
}, "If an iframe has an id and a different name, it should not be returned by its id.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[8];
  assert_equals(iframe.name, "42");

  assert_true(42 in document, '42 in document should be true');
  assert_equals(document[42], iframe.contentWindow);
}, "An iframe whose name looks like an array index should work.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[9];
  assert_equals(iframe.name, "test9");

  assert_true("test9" in document, 'test9 in document should be true');
  assert_equals(document["test9"], iframe.contentWindow);
  assert_equals(document.test9, iframe.contentWindow);

  iframe.removeAttribute("name");
  assert_false("test9" in document, 'test9 in document should be false');
  assert_equals(document["test9"], undefined);
  assert_equals(document.test9, undefined);
}, "Dynamically removing the name attribute from iframe elements, should not be accessible.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[10];
  iframe.setAttribute("name", "test10a");

  assert_true("test10a" in document, 'test10a in document should be true');
  assert_equals(document["test10a"], iframe.contentWindow);
  assert_equals(document.test10a, iframe.contentWindow);

  iframe.setAttribute("name", "test10b");
  assert_false("test10a" in document, 'test10a in document should be false');
  assert_equals(document["test10a"], undefined);
  assert_equals(document.test10a, undefined);
  assert_true("test10b" in document, 'test10b in document should be true');
  assert_equals(document["test10b"], iframe.contentWindow);
  assert_equals(document.test10b, iframe.contentWindow);
}, "Dynamically updating the name attribute from iframe elements, should be accessible by its name.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[11];
  assert_equals(iframe.name, "test11a");

  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], iframe.contentWindow);
  assert_equals(document.test11a, iframe.contentWindow);

  iframe.setAttribute("id", "test11a");
  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], iframe.contentWindow);
  assert_equals(document.test11a, iframe.contentWindow);

  iframe.setAttribute("id", "test11b");
  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], iframe.contentWindow);
  assert_equals(document.test11a, iframe.contentWindow);
  assert_false("test11b" in document, 'test11b in document should be false');
  assert_equals(document["test11b"], undefined);
  assert_equals(document.test11b, undefined);
}, "Dynamically updating the id attribute from iframe elements, should be accessible only by its name.");

test(function() {
  var iframe = document.getElementsByTagName("iframe")[12];
  assert_equals(iframe.name, "test12");

  assert_true("test12" in document, 'test12 in document should be true');
  assert_equals(document["test12"], iframe.contentWindow);
  assert_equals(document.test12, iframe.contentWindow);

  iframe.remove();
  assert_false("test12" in document, 'test12 in document should be false');
  assert_equals(document["test12"], undefined);
  assert_equals(document.test12, undefined);
}, "iframe elements that is removed, should not be accessible.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 514,
        "byte_start": 496,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 514,
        "byte_start": 496,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 534,
        "byte_start": 516,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 534,
        "byte_start": 516,
        "col": 1,
        "line": 18
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-02.html"
}
```
