# html/syntax/parsing/ambiguous-ampersand.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/ambiguous-ampersand.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Ambiguous ampersand</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div><a href='?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=&notin;&not;&;& &'>Link</a><p>Text: ?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=&notin;&not;&;& &</p></div>
<script>
var markup = "<div><a href='?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=&notin;&not;&;& &'>Link</a><p>Text: ?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=&notin;&not;&;& &</p></div>";

for (var i = 0; i < markup.length; ++i) {
    document.write(markup.charAt(i));
}
</script>

<script>
function checkDiv(div, provenance) {
    test(function() {
        assert_equals(div.childNodes.length, 2, "Number of elements " + provenance);
        let a = div.firstChild;
        let href = a.href;
        let question = href.indexOf('?');
        href = href.substring(question);
        assert_equals(href, "?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=%E2%88%89%C2%AC&;&%20&", "attribute " + provenance);
        let p = a.nextSibling;
        assert_equals(p.textContent, "Text: ?a=b&c=d&a0b=c©=1¬i=n¬=in¬in=∉¬&;& &", "text " + provenance)
    }, "Check div structure: " + provenance);
}


let divs = document.getElementsByTagName("div");
test(function() {
    assert_equals(divs.length, 2);
}, "Check number of divs");
checkDiv(divs[0], "network");
checkDiv(divs[1], "document.write");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “?a=b&c=d&a0b=c&copy=1&noti=n&not=in&notin=∉¬&;& &” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 251,
        "byte_start": 181,
        "col": 6,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.named_charref_no_semicolon",
      "message": "Named character reference was not terminated by a semicolon. (Or “&” should have been escaped as “&amp;”.)",
      "severity": "Warning",
      "span": {
        "byte_end": 283,
        "byte_start": 282,
        "col": 87,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.named_charref_no_semicolon",
      "message": "Named character reference was not terminated by a semicolon. (Or “&” should have been escaped as “&amp;”.)",
      "severity": "Warning",
      "span": {
        "byte_end": 290,
        "byte_start": 289,
        "col": 87,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.named_charref_no_semicolon",
      "message": "Named character reference was not terminated by a semicolon. (Or “&” should have been escaped as “&amp;”.)",
      "severity": "Warning",
      "span": {
        "byte_end": 297,
        "byte_start": 296,
        "col": 87,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.named_charref_no_semicolon",
      "message": "Named character reference was not terminated by a semicolon. (Or “&” should have been escaped as “&amp;”.)",
      "severity": "Warning",
      "span": {
        "byte_end": 304,
        "byte_start": 303,
        "col": 87,
        "line": 6
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
  "source_name": "html/syntax/parsing/ambiguous-ampersand.html"
}
```
