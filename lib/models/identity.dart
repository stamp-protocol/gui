import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import '../ffi.dart';
import 'dart:convert';

class IdentityModel with ChangeNotifier {
  var _identity = null;

  IdentityModel() {}

  get identity => _identity;

  Future<void> grab(String id) async {
    final String identity = await api.getBuiltIdentityById(id: id);
    final data = await json.decode(identity);
    setId(data);
  }

  void setId(identity_data) {
    _identity = identity_data;
    notifyListeners();
  }
}

