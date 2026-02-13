# html/dom/documents/dom-tree-accessors/nameditem-05.html

Counts:
- errors: 17
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-05.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: embeds</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<embed name=test1></embed>

<embed name=test2></embed>
<embed name=test2></embed>

<embed id=test3></embed>

<embed id=test4></embed>
<embed id=test4></embed>

<embed name=test5></embed>
<embed id=test5></embed>

<embed id=test6></embed>
<embed name=test6></embed>

<embed id=test7 name=fail></embed>

<embed name=test8 id=fail></embed>

<embed name="test9" id="test9"></embed>

<embed></embed>

<embed name="test11a"></embed>

<embed name="test12"></embed>
</div>
<script>
test(function() {
  var embed = document.getElementsByTagName("embed")[0];
  assert_equals(embed.name, "test1");

  assert_true("test1" in document, '"test1" in document should be true');
  assert_equals(document.test1, embed);
}, "If there is one embed, it should be returned (name)");

test(function() {
  var embed1 = document.getElementsByTagName("embed")[1];
  assert_equals(embed1.name, "test2");
  var embed2 = document.getElementsByTagName("embed")[2];
  assert_equals(embed2.name, "test2");

  assert_true("test2" in document, '"test2" in document should be true');
  var collection = document.test2;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [embed1, embed2]);
}, "If there are two embeds, a collection should be returned. (name)");

test(function() {
  var embed = document.getElementsByTagName("embed")[3];
  assert_equals(embed.id, "test3");

  assert_false("test3" in document, '"test3" in document should be false');
  assert_equals(document.test3, undefined);
}, "If there is one embed, it should not be returned (id)");

test(function() {
  var embed1 = document.getElementsByTagName("embed")[4];
  assert_equals(embed1.id, "test4");
  var embed2 = document.getElementsByTagName("embed")[5];
  assert_equals(embed2.id, "test4");

  assert_false("test4" in document, '"test4" in document should be false');
  assert_equals(document.test4, undefined);
}, "If there are two embeds, nothing should be returned. (id)");

test(function() {
  var embed1 = document.getElementsByTagName("embed")[6];
  assert_equals(embed1.name, "test5");
  var embed2 = document.getElementsByTagName("embed")[7];
  assert_equals(embed2.id, "test5");

  assert_true("test5" in document, '"test5" in document should be true');
  assert_equals(document.test5, embed1);
}, "If there are two embeds, a collection should be returned. (name and id)");

test(function() {
  var embed1 = document.getElementsByTagName("embed")[8];
  assert_equals(embed1.id, "test6");
  var embed2 = document.getElementsByTagName("embed")[9];
  assert_equals(embed2.name, "test6");

  assert_true("test6" in document, '"test6" in document should be true');
  assert_equals(document.test6, embed2);
}, "If there are two embeds, a collection should be returned. (id and name)");

test(function() {
  var embed = document.getElementsByTagName("embed")[10];
  assert_equals(embed.id, "test7");

  assert_false("test7" in document, '"test7" in document should be false');
  assert_equals(document.test7, undefined);
}, "A name shouldn't affect getting an embed by id");

test(function() {
  var embed = document.getElementsByTagName("embed")[11];
  assert_equals(embed.name, "test8");

  assert_true("test8" in document, '"test8" in document should be true');
  assert_equals(document.test8, embed);
}, "An id shouldn't affect getting an embed by name");

test(function() {
  var embed = document.getElementsByTagName("embed")[12];
  assert_equals(embed.name, "test9");

  assert_true("test9" in document, 'test9 in document should be true');
  assert_equals(document["test9"], embed);
  assert_equals(document.test9, embed);

  embed.removeAttribute("name");
  assert_false("test9" in document, 'test9 in document should be false');
  assert_equals(document["test9"], undefined);
  assert_equals(document.test9, undefined);
}, "Dynamically removing the name attribute from embed elements, should not be accessible.");

test(function() {
  var embed = document.getElementsByTagName("embed")[13];
  embed.setAttribute("name", "test10a");

  assert_true("test10a" in document, 'test10a in document should be true');
  assert_equals(document["test10a"], embed);
  assert_equals(document.test10a, embed);

  embed.setAttribute("name", "test10b");
  assert_false("test10a" in document, 'test10a in document should be false');
  assert_equals(document["test10a"], undefined);
  assert_equals(document.test10a, undefined);
  assert_true("test10b" in document, 'test10b in document should be true');
  assert_equals(document["test10b"], embed);
  assert_equals(document.test10b, embed);
}, "Dynamically updating the name attribute from embed elements, should be accessible by its name.");

test(function() {
  var embed = document.getElementsByTagName("embed")[14];
  assert_equals(embed.name, "test11a");

  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], embed);
  assert_equals(document.test11a, embed);

  embed.setAttribute("id", "test11a");
  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], embed);
  assert_equals(document.test11a, embed);

  embed.setAttribute("id", "test11b");
  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], embed);
  assert_equals(document.test11a, embed);
  assert_false("test11b" in document, 'test11b in document should be false');
  assert_equals(document["test11b"], undefined);
  assert_equals(document.test11b, undefined);
}, "Dynamically updating the id attribute from embed elements, should be accessible only by its name.");

test(function() {
  var embed = document.getElementsByTagName("embed")[15];
  assert_equals(embed.name, "test12");

  assert_true("test12" in document, 'test12 in document should be true');
  assert_equals(document["test12"], embed);
  assert_equals(document.test12, embed);

  embed.remove();
  assert_false("test12" in document, 'test12 in document should be false');
  assert_equals(document["test12"], undefined);
  assert_equals(document.test12, undefined);
}, "embed elements that is removed, should not be accessible.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 395,
        "byte_start": 387,
        "col": 19,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 423,
        "byte_start": 415,
        "col": 19,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 450,
        "byte_start": 442,
        "col": 19,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 476,
        "byte_start": 468,
        "col": 17,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 502,
        "byte_start": 494,
        "col": 17,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test4”.",
      "severity": "Error",
      "span": {
        "byte_end": 519,
        "byte_start": 503,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 527,
        "byte_start": 519,
        "col": 17,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 555,
        "byte_start": 547,
        "col": 19,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 580,
        "byte_start": 572,
        "col": 17,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 606,
        "byte_start": 598,
        "col": 17,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 633,
        "byte_start": 625,
        "col": 19,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 669,
        "byte_start": 661,
        "col": 27,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 705,
        "byte_start": 697,
        "col": 27,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 746,
        "byte_start": 738,
        "col": 32,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 763,
        "byte_start": 755,
        "col": 8,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 795,
        "byte_start": 787,
        "col": 23,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 826,
        "byte_start": 818,
        "col": 22,
        "line": 36
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-05.html"
}
```
