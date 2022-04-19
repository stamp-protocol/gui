import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import '../ffi.dart';
import 'dart:convert';

class StampModel with ChangeNotifier {
  List _identities = [];

  StampModel() {
    listLocalIdentities();
  }

  get identities => _identities;

  void clear() {
    _identities.clear();
    notifyListeners();
  }

  Future<void> listLocalIdentities() async {
    final String identities = await api.listLocalIdentitiesBuilt();
    final data = await json.decode(identities);
    setIds(data);
  }

  void setIds(List new_ids) {
    _identities = new_ids;
    notifyListeners();
  }
}
