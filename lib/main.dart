import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:stampgui/models/stamp.dart';
import 'package:stampgui/models/identity.dart';

void main() {
  runApp(const StampApp());
}

class StampApp extends StatelessWidget {
  const StampApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (context) => StampModel()),
        ChangeNotifierProvider(create: (context) => IdentityModel()),
      ],
      child: MaterialApp(
        title: 'Stamp',
        theme: ThemeData(
          primarySwatch: Colors.green,
        ),
        home: const HomePage(),
      ),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Stamp'),
      ),
      drawer: Drawer(
        child: ListView(
          children: [
            ListTile(
              title: const Text('Things'),
            ),
            ListTile(
              title: const Text('Stuff'),
            ),
          ],
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.all(25),
        child: Column(
          children: [
            Expanded(
              child: Consumer<StampModel>(
                builder: (context, model, child) => ListView.builder(
                  itemCount: model.identities.length,
                  itemBuilder: (context, index) {
                    final identity = model.identities[index];
                    final identity_id = identity['id']['Ed25519'];
                    final identity_id_short = identity_id.substring(0, 16);
                    return Card(
                      margin: const EdgeInsets.all(10),
                      child: ListTile(
                        leading: Text(identity_id_short),
                        title: Text('Noice'),
                        subtitle: Text('A brand new dance'),
                        onTap: () {
                          Navigator.push(
                            context,
                            MaterialPageRoute(builder: (context) => IdentityPage(identity_id: identity_id)),
                          );
                        },
                      ),
                    );
                  },
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class IdentityPage extends StatelessWidget {
  const IdentityPage({Key? key, @required this.identity_id}) : super(key: key);
  final identity_id;

  @override
  Widget build(BuildContext context) {
    Provider.of<IdentityModel>(context).grab(identity_id);
    return Scaffold(
      appBar: AppBar(
        title: Text('Identity: ${identity_id.substring(0, 16)}'),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(25),
          child: Consumer<IdentityModel>(
            builder: (context, model, _) {
              if(model.identity == null) return const CircularProgressIndicator();
              return Text('get a job ${model.identity["created"]}');
            },
          ),
        ),
      ),
    );
  }
}
/*
FutureBuilder<List<dynamic>>(
                future: Future.wait([listLocalIdentities()]),
                builder: (context, snap) {
                  if (snap.error != null) {
                    debugPrint(snap.error.toString());
                    return Tooltip(
                      message: snap.error.toString(),
                      child: Text('Unknown OS', style: style),
                    );
                  }

                  final data = snap.data;
                  if (data == null) return const CircularProgressIndicator();

                  return ListView.builder(
                      itemCount: ids.ids.length,
                      itemBuilder: (context, index) {
                      final identity = ids.ids[index];
                      final identity_id =
                      identity['id']['Ed25519'].substring(0, 16);
                      return Card(
                          margin: const EdgeInsets.all(10),
                          child: ListTile(
                            leading: Text(identity_id),
                            title: Text('Noice'),
                            subtitle: Text('A brand new dance'),
                            onTap: () {
                            Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (context) =>
                                  IdentityPage(title: identity_id)),
                                );
                            }),
                          );
                      },
                      )

                  final String identities = data[0];
                  return Text(identities, style: style);
                },
              )
*/
