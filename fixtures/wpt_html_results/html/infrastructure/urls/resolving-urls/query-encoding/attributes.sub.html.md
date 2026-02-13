# html/infrastructure/urls/resolving-urls/query-encoding/attributes.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/resolving-urls/query-encoding/attributes.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset={{GET[encoding]}}> <!-- ends up as <meta charset> by default which is windows-1252 -->
<meta name=variant content="?encoding=windows-1252">
<meta name=variant content="?encoding=x-cp1251">
<meta name=variant content="?encoding=utf8">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<link rel=help href=https://html.spec.whatwg.org/multipage/rendering.html#the-page>
<link rel=help href=https://html.spec.whatwg.org/multipage/rendering.html#tables-2>
<link rel=help href=https://html.spec.whatwg.org/multipage/#reflecting-content-attributes-in-idl-attributes>
<div id=log></div>
<script>
function expected(encoding) {
  return "?" + {
    "UTF-8": "%C3%BF",
    "windows-1251": "%26%23255%3B",
    "windows-1252": "%FF"
  }[encoding];
}

function assert_ends_with(input, endsWith) {
  assert_true(input.endsWith(endsWith), input + " did not end with " + endsWith);
}

"body table thead tbody tfoot tr td th".split(" ").forEach(localName => {
  test(t => {
    const elm = document.createElement(localName);
    document.body.appendChild(elm);
    t.add_cleanup(() => { document.body.removeChild(elm); });
    elm.setAttribute("background", "?\u00FF");
    assert_ends_with(getComputedStyle(elm).backgroundImage, expected(document.characterSet) + "\")");
  }, "getComputedStyle <" + localName + " background>");
});

function test_url_reflecting(localName, attr, idlAttr) {
  idlAttr = idlAttr || attr;
  test(() => {
    let input = "?\u00FF";
    const elm = document.createElement(localName);
    assert_true(idlAttr in elm, idlAttr + " is not supported");
    elm.setAttribute(attr, input);
    assert_ends_with(elm[idlAttr], expected(document.characterSet));
  }, "Getting <" + localName + ">." + idlAttr);
}

("iframe src, a href, area href, base href, link href, img src, embed src, object data, " +
 "track src, video src, audio src, input src, form action, input formaction formAction, " +
 "button formaction formAction, script src").split(", ").forEach(str => {
  const arr = str.split(" ");
  test_url_reflecting(arr[0], arr[1], arr[2]);
});

function test_string_reflecting(localName, attr) {
  test(() => {
    let input = "?\u00FF ?\u00FF";
    const elm = document.createElement(localName);
    assert_true(attr in elm, attr + " is not supported");
    elm.setAttribute(attr, input);
    assert_equals(elm[attr], input);
  }, "Getting <" + localName + ">." + attr);
}

"a ping, area ping".split(", ").forEach(str => {
  const arr = str.split(" ");
  test_string_reflecting(arr[0], arr[1]);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “{{GET[encoding]}}” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 48,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/infrastructure/urls/resolving-urls/query-encoding/attributes.sub.html"
}
```
