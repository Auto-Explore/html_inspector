# html/dom/documents/dom-tree-accessors/nameditem-07.html

Counts:
- errors: 1
- warnings: 16
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-07.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: objects</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/dom.html#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<object name=test1></object>

<object name=test2></object>
<object name=test2></object>

<object id=test3></object>

<object id=test4></object>
<object id=test4></object>

<object name=test5></object>
<object id=test5></object>

<object id=test6></object>
<object name=test6></object>

<object name=test7></object>

<object id=test8></object>

<object id=test9a></object>

<object name=test10a></object>

<object name=test11a id=test11b></object>
</div>
<script>
test(function() {
  var object = document.getElementsByTagName("object")[0];
  assert_equals(object.name, "test1");

  assert_true("test1" in document, '"test1" in document should be true');
  assert_equals(document.test1, object);
}, "If there is one object, it should be returned (name)");

test(function() {
  var object1 = document.getElementsByTagName("object")[1];
  assert_equals(object1.name, "test2");
  var object2 = document.getElementsByTagName("object")[2];
  assert_equals(object2.name, "test2");

  assert_true("test2" in document, '"test2" in document should be true');
  var collection = document.test2;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [object1, object2]);
}, "If there are two objects, a collection should be returned. (name)");

test(function() {
  var object = document.getElementsByTagName("object")[3];
  assert_equals(object.id, "test3");

  assert_true("test3" in document, '"test3" in document should be true');
  assert_equals(document.test3, object);
}, "If there is one object, it should be returned (id)");

test(function() {
  var object1 = document.getElementsByTagName("object")[4];
  assert_equals(object1.id, "test4");
  var object2 = document.getElementsByTagName("object")[5];
  assert_equals(object2.id, "test4");

  assert_true("test4" in document, '"test4" in document should be true');
  var collection = document.test4;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [object1, object2]);
}, "If there are two objects, a collection should be returned. (id)");

test(function() {
  var object1 = document.getElementsByTagName("object")[6];
  assert_equals(object1.name, "test5");
  var object2 = document.getElementsByTagName("object")[7];
  assert_equals(object2.id, "test5");

  assert_true("test5" in document, '"test5" in document should be true');
  var collection = document.test5;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [object1, object2]);
}, "If there are two objects, a collection should be returned. (name and id)");

test(function() {
  var object1 = document.getElementsByTagName("object")[8];
  assert_equals(object1.id, "test6");
  var object2 = document.getElementsByTagName("object")[9];
  assert_equals(object2.name, "test6");

  assert_true("test6" in document, '"test6" in document should be true');
  var collection = document.test6;
  assert_class_string(collection, "HTMLCollection", "collection should be an HTMLCollection");
  assert_array_equals(collection, [object1, object2]);
}, "If there are two objects, a collection should be returned. (id and name)");

test(function() {
  var object = document.getElementsByTagName("object")[10];
  assert_equals(object.name, "test7");

  assert_true("test7" in document, 'test7 in document should be true');
  assert_equals(document["test7"], object);
  assert_equals(document.test7, object);

  object.removeAttribute("name");
  assert_false("test7" in document, 'test7 in document should be false');
  assert_equals(document["test7"], undefined);
  assert_equals(document.test7, undefined);
}, "Dynamically removing the name attribute from object elements, should not be accessible.");

test(function() {
  var object = document.getElementsByTagName("object")[11];
  assert_equals(object.id, "test8");

  assert_true("test8" in document, 'test8 in document should be true');
  assert_equals(document["test8"], object);
  assert_equals(document.test8, object);

  object.removeAttribute("id");
  assert_false("test8" in document, 'test8 in document should be false');
  assert_equals(document["test8"], undefined);
  assert_equals(document.test8, undefined);
}, "Dynamically removing the id attribute from object elements, should not be accessible.");

test(function() {
  var object = document.getElementsByTagName("object")[12];
  assert_equals(object.id, "test9a");
  assert_true("test9a" in document, 'test9a in document should be true');
  assert_equals(document["test9a"], object);
  assert_equals(document.test9a, object);

  object.setAttribute("name", "test9a");
  assert_true("test9a" in document, 'test9a in document should be true');
  assert_equals(document["test9a"], object);
  assert_equals(document.test9a, object);

  object.setAttribute("name", "test9b");
  assert_true("test9a" in document, 'test9a in document should be true');
  assert_equals(document["test9a"], object);
  assert_equals(document.test9a, object);
  assert_true("test9b" in document, 'test9b in document should be true');
  assert_equals(document["test9b"], object);
  assert_equals(document.test9b, object);

  object.setAttribute("name", "test9c");
  assert_true("test9a" in document, 'test9a in document should be true');
  assert_equals(document["test9a"], object);
  assert_equals(document.test9a, object);
  assert_false("test9b" in document, 'test9b in document should be false');
  assert_equals(document["test9b"], undefined);
  assert_equals(document.test9b, undefined);
  assert_true("test9c" in document, 'test9c in document should be true');
  assert_equals(document["test9c"], object);
  assert_equals(document.test9c, object);
}, "Dynamically updating the name attribute from object elements, should be accessible by its name and id.");

test(function() {
  var object = document.getElementsByTagName("object")[13];
  assert_equals(object.name, "test10a");
  assert_true("test10a" in document, 'test11a in document should be true');
  assert_equals(document["test10a"], object);
  assert_equals(document.test10a, object);

  object.setAttribute("id", "test10a");
  assert_true("test10a" in document, 'test10a in document should be true');
  assert_equals(document["test10a"], object);
  assert_equals(document.test10a, object);

  object.setAttribute("id", "test10b");
  assert_true("test10a" in document, 'test10a in document should be true');
  assert_equals(document["test10a"], object);
  assert_equals(document.test10a, object);
  assert_true("test10b" in document, 'test10b in document should be false');
  assert_equals(document["test10b"], object);
  assert_equals(document.test10b, object);

  object.setAttribute("id", "test10c");
  assert_true("test10a" in document, 'test10a in document should be true');
  assert_equals(document["test10a"], object);
  assert_equals(document.test10a, object);
  assert_false("test10b" in document, 'test10b in document should be false');
  assert_equals(document["test10b"], undefined);
  assert_equals(document.test10b, undefined);
  assert_true("test10c" in document, 'test10b in document should be false');
  assert_equals(document["test10c"], object);
  assert_equals(document.test10c, object);
}, "Dynamically updating the id attribute from object elements, should be accessible by its name and id.");

test(function() {
  var object = document.getElementsByTagName("object")[14];
  assert_equals(object.name, "test11a");
  assert_equals(object.id, "test11b");

  assert_true("test11a" in document, 'test11a in document should be true');
  assert_equals(document["test11a"], object);
  assert_equals(document.test11a, object);
  assert_true("test11b" in document, 'test11b in document should be true');
  assert_equals(document["test11b"], object);
  assert_equals(document.test11b, object);

  object.remove();
  assert_false("test11a" in document, 'test11a in document should be false');
  assert_equals(document["test11a"], undefined);
  assert_equals(document.test11a, undefined);
  assert_false("test11b" in document, 'test11b in document should be false');
  assert_equals(document["test11b"], undefined);
  assert_equals(document.test11b, undefined);
}, "object elements that is removed, should not be accessible.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 331,
        "byte_start": 312,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 361,
        "byte_start": 342,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 390,
        "byte_start": 371,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 418,
        "byte_start": 401,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 429,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “test4”.",
      "severity": "Error",
      "span": {
        "byte_end": 473,
        "byte_start": 456,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 473,
        "byte_start": 456,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 503,
        "byte_start": 484,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 530,
        "byte_start": 513,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 558,
        "byte_start": 541,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 587,
        "byte_start": 568,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 617,
        "byte_start": 598,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 645,
        "byte_start": 628,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 674,
        "byte_start": 656,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 706,
        "byte_start": 685,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 749,
        "byte_start": 717,
        "col": 1,
        "line": 33
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-07.html"
}
```
