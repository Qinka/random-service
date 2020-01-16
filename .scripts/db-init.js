// use rndsvr

db.token.deleteMany({});
db.gp.deleteMany({});
db.gp.insertOne({name: "all",       auths: ["all", 'anonymous']});
db.gp.insertOne({name: "anonymous", auths: ["anonymous"]});