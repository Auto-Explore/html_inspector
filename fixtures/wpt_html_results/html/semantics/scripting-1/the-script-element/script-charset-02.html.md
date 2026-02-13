# html/semantics/scripting-1/the-script-element/script-charset-02.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-charset-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Script encoding for document encoding windows-1250</title>
  <link rel="author" title="askalski" href="github.com/askalski">
  <link rel="author" title="Aaqa Ishtyaq" href="github.com/aaqaishtyaq">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#fetch-a-classic-script">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <div id="log"></div>
  <!-- to avoid conflating tests for script encoding declaring the encoding at the top of file. i.e, windows-1250-->
  <meta charset="windows-1250">
  <script>
  test(function() {
    assert_equals(document.characterSet, "windows-1250")
  }, "assumption: document encoding is windows-1250");
  </script>

  <!-- in this case, neither response's Content Type nor charset attribute bring correct charset information.
  -->
  <script type="text/javascript"
    src="serve-with-content-type.py?fn=external-script-windows1250.js&ct=text/javascript">
  </script>

  <script>
  test(function() {
    //these string should match since, windows-1250 is the fallback encoding.
    assert_equals(window.getSomeString(), "\u015b\u0107\u0105\u017c\u017a");
  }, "windows-1250 script decoded using document encoding (also windows-1250)");
  </script>

  <script type="text/javascript"
    src="serve-with-content-type.py?fn=external-script-utf8.js&ct=text/javascript">
  </script>
  <script>
  //these strings should match, since this string is the result of decoding the utf-8 text as windows-1250.
  test(function() {
    assert_equals(window.getSomeString(), "\u0139\u203a\xc4\u2021\xc4\u2026\u0139\u013d\u0139\u015f");
  }, "UTF-8 script decoded using document encoding (windows-1250)");
  </script>

</head>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 1766,
        "byte_start": 1759,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1250” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 616,
        "byte_start": 587,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1012,
        "byte_start": 891,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1422,
        "byte_start": 1308,
        "col": 3,
        "line": 31
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-charset-02.html"
}
```
