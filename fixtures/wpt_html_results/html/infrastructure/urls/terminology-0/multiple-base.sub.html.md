# html/infrastructure/urls/terminology-0/multiple-base.sub.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/terminology-0/multiple-base.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>document base URL: multiple base elements</title>
<base target="_blank" >
<base href="http://{{domains[www]}}:{{ports[http][0]}}/">
<base href="http://{{domains[www1]}}:{{ports[http][0]}}/">
<base href="http://{{domains[www2]}}:{{ports[http][0]}}/">
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  test(function(){
    var base = document.querySelectorAll("base");
    assert_equals(document.baseURI, document.querySelectorAll("base[href]")[0].href);
  }, "If there are multiple <base> elements, the document base URL is the frozen base URL of the first one that has an href attribute");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.href.invalid",
      "message": "Bad value “http://{{domains[www]}}:{{ports[http][0]}}/” for attribute “href” on element “base”.",
      "severity": "Warning",
      "span": {
        "byte_end": 177,
        "byte_start": 120,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.base.href.invalid",
      "message": "Bad value “http://{{domains[www1]}}:{{ports[http][0]}}/” for attribute “href” on element “base”.",
      "severity": "Warning",
      "span": {
        "byte_end": 236,
        "byte_start": 178,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.base.href.invalid",
      "message": "Bad value “http://{{domains[www2]}}:{{ports[http][0]}}/” for attribute “href” on element “base”.",
      "severity": "Warning",
      "span": {
        "byte_end": 295,
        "byte_start": 237,
        "col": 1,
        "line": 7
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
  "source_name": "html/infrastructure/urls/terminology-0/multiple-base.sub.html"
}
```
