use futures::StreamExt;
use mongodb::{Collection, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DbCollection<T>
where
    T: Clone,
{
    collection: Collection,
    phantom: PhantomData<T>,
}

impl<'de, T> DbCollection<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub fn new(db: &Database) -> DbCollection<T> {
        let mut tname = std::any::type_name::<T>().to_lowercase();
        tname.push('s');
        let words: Vec<&str> = tname.split("::").collect();
        let name = words[words.len() - 1];
        Self {
            collection: db.collection(name),
            phantom: PhantomData,
        }
    }
    pub async fn index(&self) -> Vec<T> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut response: Vec<T> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let doc = doc.unwrap();
            let val: T = bson::from_document(doc).unwrap();
            response.push(val);
        }
        response
    }

    pub async fn find(&self, query: bson::Document) -> Vec<T> {
        let mut cursor = self.collection.find(query, None).await.unwrap();
        let mut response: Vec<T> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let doc = doc.unwrap();
            let val: T = bson::from_document(doc).unwrap();
            response.push(val);
        }
        response
    }
    pub async fn find_by_id(&self, id: bson::oid::ObjectId) -> Result<T, mongodb::error::Error> {
        let doc = bson::doc! {"_id":id};
        let res = self.collection.find_one(doc, None).await?;
        let data = bson::from_document(res.unwrap()).expect(
            format! {"Failed to parse {} from Document",std::any::type_name::<T>()}.as_str(),
        );
        Ok(data)
    }
    pub async fn count(&self, query: Option<bson::Document>) -> Result<i64, mongodb::error::Error> {
        let res = self.collection.count_documents(query, None).await?;
        Ok(res)
    }
    pub async fn create(&self, data: T) -> Result<bson::Bson, mongodb::error::Error> {
        let doc = bson::to_document(&data)
            .expect(format! {"Failed to parse {} to Document",std::any::type_name::<T>()}.as_str());
        let res = self.collection.insert_one(doc, None).await?;
        println!("{}", res.inserted_id);
        Ok(res.inserted_id)
    }

    pub async fn delete_one(&self, query: bson::Document) -> Result<i64, mongodb::error::Error> {
        let res = self.collection.delete_one(query, None).await?;
        Ok(res.deleted_count)
    }

    pub async fn delete(&self, query: bson::Document) -> Result<i64, mongodb::error::Error> {
        let res = self.collection.delete_many(query, None).await?;
        Ok(res.deleted_count)
    }

    pub async fn delete_by_id(
        &self,
        id: bson::oid::ObjectId,
    ) -> Result<i64, mongodb::error::Error> {
        let doc = bson::doc! {"_id":id};
        self.delete_one(doc).await
    }
}
