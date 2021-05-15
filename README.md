# speechdis

A webapp to transcribe some audio content from your favorite podcast or radio show using the redisAI Rust client build also for this hackathon.

Based on recent advances in speech to text some new Machine Learning model can transcribe speech from raw audio into text. This Webapp used trained model uploaded into a Redis instance with the module RedisAI. The transcribed text is then also store in redis with the module redis search in order to easily search in the transcript some information.
