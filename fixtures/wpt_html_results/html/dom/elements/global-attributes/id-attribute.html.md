# html/dom/elements/global-attributes/id-attribute.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/id-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
<title>The id attribute</title>
<meta charset=utf8>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-id-attribute">
<style>

#abcd {
   position: absolute;
   z-index: 1;
}

#ABCD {
   position: absolute;
   z-index: 2;
}

#a\ b {
   position: absolute;
   z-index: 3;
}

#xyz {
   position: absolute;
   z-index: 4;
}

#foobar {
   position: absolute;
   z-index: 5;
}

#åèiöú {
   position: absolute;
   z-index: 6;
}

</style>
</head>
<body>
<h1>The id attribute</h1>
<div id="log"></div>
<i id="abcd"></i>
<i id="ABCD"></i>
<i id="a b"></i>
<i id="åèiöú"></i>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    // id is associated for purposes of getElementById
    test(function() {
        assert_equals(document.getElementById("abcd"), document.getElementsByTagName("i")[0]);
    }, "User agents must associate the element with an id value for purposes of getElementById.");

    test(function() {
        assert_equals(document.getElementById("ABCD"), document.getElementsByTagName("i")[1]);
    }, "Association is exact and therefore case-sensitive for getElementById.");

    test(function() {
        assert_equals(document.getElementById("a b"), document.getElementsByTagName("i")[2]);
    }, "Spaces are allowed in an id and still make an association for getElementByID.");

    test(function() {
        assert_equals(document.getElementById("åèiöú"), document.getElementsByTagName("i")[3]);
    }, "Non-ASCII is allowed in an id and still make an association for getElementById.");


    // id is associated for purposes of CSS
    test(function() {
        assert_equals(document.defaultView.getComputedStyle(document.getElementById("abcd")).zIndex, "1");
    }, "User agents must associate the element with an id value for purposes of CSS.");

    test(function() {
        assert_equals(document.defaultView.getComputedStyle(document.getElementById("ABCD")).zIndex, "2");
    }, "Association for CSS is exact and therefore case-sensitive.");

    test(function() {
        assert_equals(document.defaultView.getComputedStyle(document.getElementById("a b")).zIndex, "3");
    }, "Spaces are allowed in an id and still make an association.");

    test(function() {
        assert_equals(document.defaultView.getComputedStyle(document.getElementById("åèiöú")).zIndex, "6");
    }, "Non-ASCII is allowed in an id and still make an association for CSS.");


    // id IDL attribute reflects the content attribute
    var firstSpan = document.getElementById("abcd");

    test(function() {
        assert_equals(firstSpan.id, "abcd");
    }, "The id IDL attribute must reflect the id content attribute, for getting.");

    test(function() {
        firstSpan.id = "xyz";
        assert_equals(firstSpan.getAttribute("id"), "xyz");
    }, "The id IDL attribute must reflect the id content attribute, for setting via IDL attribute.");

    test(function() {
        assert_equals(document.getElementById("xyz"), firstSpan);
    }, "After setting id via id attribute, getElementById find the element by the new id.");

    test(function() {
        assert_equals(document.getElementById("abcd"), null);
    }, "After setting id via id attribute, getElementById doesn't find the element by the old id.");

    test(function() {
        assert_equals(document.defaultView.getComputedStyle(firstSpan).zIndex, "4");
    }, "After setting id via id attribute, CSS association is via the new ID.");

    test(function() {
        firstSpan.setAttribute("id", "foobar");
        assert_equals(firstSpan.id, "foobar");
    }, "The id IDL attribute must reflect the id content attribute, for setting via setAttribute.");

    test(function() {
        assert_equals(document.getElementById("foobar"), firstSpan);
    }, "After setting id via setAttribute attribute, getElementById find the element by the new id.");

    test(function() {
        assert_equals(document.getElementById("xyz"), null);
    }, "After setting id via setAttribute attribute, getElementById doesn't find the element by the old id.");

    test(function() {
        assert_equals(document.defaultView.getComputedStyle(firstSpan).zIndex, "5");
    }, "After setting id via setAttribute attribute, CSS association is via the new ID.");

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “a b” for attribute “id” on element “i”.",
      "severity": "Warning",
      "span": {
        "byte_end": 592,
        "byte_start": 580,
        "col": 1,
        "line": 46
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
  "source_name": "html/dom/elements/global-attributes/id-attribute.html"
}
```
