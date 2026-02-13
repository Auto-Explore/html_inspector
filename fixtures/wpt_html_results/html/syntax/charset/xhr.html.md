# html/syntax/charset/xhr.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/xhr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="windows-1253">
<title>meta charset in XHR</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<script>
setup({explicit_done:true});
window.onload = function() {
  runNextTest();
};

let files = [
  "after-1kb.html",
  "after-bogus-after-1kb.html",
  "after-bogus.html",
  "after-head-after-1kb-crlf.html",
  "after-head-after-1kb.html",
  "after-head-in-1kb-crlf.html",
  "after-head-in-1kb.html",
  "baseline.html",
  "document-write.html",
  "in-comment.html",
  "in-noscript-after-template-after-1kb.html",
  "in-object.html",
  "in-script.html",
  "in-style.html",
  "in-svg.html",
  "in-svg-in-cdata.html",
  "in-template-after-1kb.html",
  "in-template.html",
  "in-title.html",
  "ncr.html",
  "non-ascii-in-comment-before.html",
  "non-ascii-in-title-before.html",
];

function handler() {
  const prefix = "Test: ";
  let doc = this.response;
  test(function() {
    let link = doc.getElementsByTagName("link")[0];
    let match = (link.rel == "match");
    let content = doc.documentElement.textContent;
    let index = content.indexOf(prefix);
    let c = content.substring(index + prefix.length, index + prefix.length + 1);
    if (match) {
      assert_equals(doc.characterSet, "windows-1251", 'Check characterSet');
      assert_equals(c, "\u0436", 'Check character');
    } else {
      assert_equals(doc.characterSet, "UTF-8", 'Check characterSet');
      assert_equals(c, "\uFFFD", 'Check character');
    }
  }, "Check " + this.thesrc);
  runNextTest();
}

function runNextTest() {
  let file = files.shift();
  if (!file) {
    done();
    return;
  }
  let xhr = new XMLHttpRequest();
  xhr.responseType = "document";
  xhr.onload = handler;
  xhr.thesrc = file; // expando
  xhr.open("GET", file);
  xhr.send();
}

</script>

```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1253” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 45,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/syntax/charset/xhr.html"
}
```
